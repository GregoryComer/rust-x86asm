use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 193, 103], OperandSize::Dword)
}

#[test]
fn vpinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1040273982, Some(OperandSize::Word), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 196, 180, 193, 62, 82, 1, 62, 112], OperandSize::Dword)
}

#[test]
fn vpinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 196, 244, 60], OperandSize::Qword)
}

#[test]
fn vpinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 6, 46], OperandSize::Qword)
}

#[test]
fn vpinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(126)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 196, 193, 126], OperandSize::Dword)
}

#[test]
fn vpinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 196, 27, 54], OperandSize::Dword)
}

#[test]
fn vpinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 1, 196, 207, 91], OperandSize::Qword)
}

#[test]
fn vpinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM24)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Two, Some(OperandSize::Word), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 61, 0, 196, 28, 123, 81], OperandSize::Qword)
}

