use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fcom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 19], OperandSize::Word)
}

fn fcom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(EAX, ECX, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 72], OperandSize::Dword)
}

fn fcom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 20, 243], OperandSize::Qword)
}

fn fcom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 214], OperandSize::Word)
}

fn fcom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 211], OperandSize::Dword)
}

fn fcom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(Direct(ST7)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 215], OperandSize::Qword)
}

fn fcom_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 217, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 144, 217, 0], OperandSize::Word)
}

fn fcom_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 20, 120], OperandSize::Dword)
}

fn fcom_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FCOM, operand1: Some(IndirectDisplaced(RAX, 950591919, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 144, 175, 225, 168, 56], OperandSize::Qword)
}

