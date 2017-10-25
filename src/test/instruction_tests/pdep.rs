use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pdep_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 245, 203], OperandSize::Dword)
}

fn pdep_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 1144405453, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 83, 245, 180, 186, 205, 61, 54, 68], OperandSize::Dword)
}

fn pdep_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 67, 245, 211], OperandSize::Qword)
}

fn pdep_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1661329159, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 107, 245, 20, 133, 7, 223, 5, 99], OperandSize::Qword)
}

fn pdep_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RBX)), operand2: Some(Direct(RCX)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 243, 245, 219], OperandSize::Qword)
}

fn pdep_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PDEP, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: Some(IndirectDisplaced(RDI, 1075054856, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 243, 245, 143, 8, 9, 20, 64], OperandSize::Qword)
}

