use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 238, 114], OperandSize::Dword)
}

#[test]
fn vpinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Four, Some(OperandSize::Word), None)), operand4: Some(Literal8(90)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 196, 28, 136, 90], OperandSize::Dword)
}

#[test]
fn vpinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 196, 252, 34], OperandSize::Qword)
}

#[test]
fn vpinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 196, 1, 121], OperandSize::Qword)
}

#[test]
fn vpinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(100)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 196, 223, 100], OperandSize::Dword)
}

#[test]
fn vpinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1239546343, Some(OperandSize::Word), None)), operand4: Some(Literal8(120)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 196, 136, 231, 249, 225, 73, 120], OperandSize::Dword)
}

#[test]
fn vpinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 77, 8, 196, 206, 22], OperandSize::Qword)
}

#[test]
fn vpinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RDI, 474997282, Some(OperandSize::Word), None)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 169, 196, 167, 34, 226, 79, 28, 110], OperandSize::Qword)
}

