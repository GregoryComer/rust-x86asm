use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn andn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 255], OperandSize::Dword)
}

fn andn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 55], OperandSize::Dword)
}

fn andn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 96, 242, 227], OperandSize::Qword)
}

fn andn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 242, 16], OperandSize::Qword)
}

fn andn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: Some(Direct(RBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 200, 242, 221], OperandSize::Qword)
}

fn andn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDN, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: Some(IndirectDisplaced(RDX, 351002749, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 224, 242, 186, 125, 224, 235, 20], OperandSize::Qword)
}

