use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sete_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 193], OperandSize::Word)
}

fn sete_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 4], OperandSize::Word)
}

fn sete_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 194], OperandSize::Dword)
}

fn sete_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 4, 177], OperandSize::Dword)
}

fn sete_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 194], OperandSize::Qword)
}

fn sete_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(IndirectScaledDisplaced(RDX, Two, 499798432, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 4, 85, 160, 81, 202, 29], OperandSize::Qword)
}

fn sete_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 193], OperandSize::Qword)
}

fn sete_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETE, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 148, 6], OperandSize::Qword)
}

