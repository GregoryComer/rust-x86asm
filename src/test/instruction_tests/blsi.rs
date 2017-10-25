use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn blsi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 88, 243, 217], OperandSize::Dword)
}

fn blsi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(EBP)), operand2: Some(IndirectDisplaced(ESI, 485857788, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 80, 243, 158, 252, 153, 245, 28], OperandSize::Dword)
}

fn blsi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 112, 243, 217], OperandSize::Qword)
}

fn blsi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(ESI)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 72, 243, 30], OperandSize::Qword)
}

fn blsi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 240, 243, 219], OperandSize::Qword)
}

fn blsi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BLSI, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 192, 243, 31], OperandSize::Qword)
}

