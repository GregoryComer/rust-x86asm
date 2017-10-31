use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn aeskeygenassist_1() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 224, 54], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_2() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 1947552624, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 20, 245, 112, 75, 21, 116, 9], OperandSize::Dword)
}

#[test]
fn aeskeygenassist_3() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 212, 83], OperandSize::Qword)
}

#[test]
fn aeskeygenassist_4() {
    run_test(&Instruction { mnemonic: Mnemonic::AESKEYGENASSIST, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 1703858931, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 223, 139, 243, 210, 142, 101, 108], OperandSize::Qword)
}

