use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 226, 1], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ECX, 963961251, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 137, 163, 225, 116, 57, 62], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 233, 45], OperandSize::Qword)
}

#[test]
fn aeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 1635140747, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 155, 139, 68, 118, 97, 82], OperandSize::Qword)
}

