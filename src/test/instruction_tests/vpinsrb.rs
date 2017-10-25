use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpinsrb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(ECX)), operand4: Some(Literal8(117)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 32, 233, 117], OperandSize::Dword)
}

fn vpinsrb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1260715425, Some(OperandSize::Byte), None)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 32, 188, 113, 161, 253, 36, 75, 39], OperandSize::Dword)
}

fn vpinsrb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESP)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 32, 228, 17], OperandSize::Qword)
}

fn vpinsrb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RDI, 53419628, Some(OperandSize::Byte), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 32, 151, 108, 30, 47, 3, 54], OperandSize::Qword)
}

fn vpinsrb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(EBX)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 32, 195, 57], OperandSize::Dword)
}

fn vpinsrb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand4: Some(Literal8(61)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 32, 38, 61], OperandSize::Dword)
}

fn vpinsrb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESI)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 99, 113, 32, 222, 122], OperandSize::Qword)
}

fn vpinsrb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPINSRB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Byte), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 25, 32, 63, 66], OperandSize::Qword)
}

