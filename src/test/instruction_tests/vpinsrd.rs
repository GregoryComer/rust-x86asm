use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpinsrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 97, 34, 198, 23], OperandSize::Dword)
}

fn vpinsrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 34, 15, 61], OperandSize::Dword)
}

fn vpinsrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 34, 231, 117], OperandSize::Qword)
}

fn vpinsrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 34, 20, 81, 91], OperandSize::Qword)
}

fn vpinsrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(EDI)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 34, 247, 14], OperandSize::Dword)
}

fn vpinsrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1662251434, Some(OperandSize::Dword), None)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 34, 52, 133, 170, 241, 19, 99, 95], OperandSize::Dword)
}

fn vpinsrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 0, 34, 252, 61], OperandSize::Qword)
}

fn vpinsrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 99, 125, 8, 34, 52, 254, 43], OperandSize::Qword)
}

