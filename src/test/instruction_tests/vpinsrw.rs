use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 196, 201, 94], OperandSize::Dword)
}

fn vpinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1220010107, Some(OperandSize::Word), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 196, 52, 213, 123, 224, 183, 72, 108], OperandSize::Dword)
}

fn vpinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 196, 244, 75], OperandSize::Qword)
}

fn vpinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Word), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 12, 66, 79], OperandSize::Qword)
}

fn vpinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 196, 199, 55], OperandSize::Dword)
}

fn vpinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1442083484, Some(OperandSize::Word), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 196, 156, 114, 156, 114, 244, 85, 89], OperandSize::Dword)
}

fn vpinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(35)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 0, 196, 196, 35], OperandSize::Qword)
}

fn vpinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Word), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 0, 196, 28, 118, 101], OperandSize::Qword)
}

