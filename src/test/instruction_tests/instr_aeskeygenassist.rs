use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 236, 31], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 20, 152, 6], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 217, 65], OperandSize::Qword)
}

#[test]
fn aeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1377783110, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 188, 242, 70, 77, 31, 82, 86], OperandSize::Qword)
}

