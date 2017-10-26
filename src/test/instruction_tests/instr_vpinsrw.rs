use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 233, 3], OperandSize::Dword)
}

#[test]
fn vpinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 855918384, Some(OperandSize::Word), None)), operand4: Some(Literal8(74)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 196, 180, 136, 48, 71, 4, 51, 74], OperandSize::Dword)
}

#[test]
fn vpinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 196, 198, 78], OperandSize::Qword)
}

#[test]
fn vpinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 374550764, Some(OperandSize::Word), None)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 196, 154, 236, 48, 83, 22, 35], OperandSize::Qword)
}

#[test]
fn vpinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 196, 198, 64], OperandSize::Dword)
}

#[test]
fn vpinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 269231693, Some(OperandSize::Word), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 196, 156, 79, 77, 38, 12, 16, 116], OperandSize::Dword)
}

#[test]
fn vpinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 185, 196, 233, 22], OperandSize::Qword)
}

#[test]
fn vpinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Word), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 45, 0, 196, 36, 95, 78], OperandSize::Qword)
}

