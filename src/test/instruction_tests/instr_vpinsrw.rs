use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(EBX)), operand4: Some(Literal8(27)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 196, 203, 27], OperandSize::Dword)
}

#[test]
fn vpinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 2135058042, Some(OperandSize::Word), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 196, 175, 122, 102, 66, 127, 118], OperandSize::Dword)
}

#[test]
fn vpinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(EBP)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 253, 116], OperandSize::Qword)
}

#[test]
fn vpinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RSI, 692811173, Some(OperandSize::Word), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 196, 190, 165, 117, 75, 41, 93], OperandSize::Qword)
}

#[test]
fn vpinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 196, 220, 33], OperandSize::Dword)
}

#[test]
fn vpinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1911028851, Some(OperandSize::Word), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 196, 60, 253, 115, 252, 231, 113, 127], OperandSize::Dword)
}

#[test]
fn vpinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 93, 0, 196, 254, 94], OperandSize::Qword)
}

#[test]
fn vpinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 2051743249, Some(OperandSize::Word), None)), operand4: Some(Literal8(45)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 5, 0, 196, 172, 200, 17, 30, 75, 122, 45], OperandSize::Qword)
}

