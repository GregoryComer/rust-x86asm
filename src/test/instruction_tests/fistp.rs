use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fistp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 31], OperandSize::Word)
}

fn fistp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 26], OperandSize::Dword)
}

fn fistp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 30], OperandSize::Qword)
}

fn fistp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(DI, 85, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 93, 85], OperandSize::Word)
}

fn fistp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 28, 145], OperandSize::Dword)
}

fn fistp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(RBX, 827824404, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 155, 20, 153, 87, 49], OperandSize::Qword)
}

fn fistp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 63], OperandSize::Word)
}

fn fistp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 59], OperandSize::Dword)
}

fn fistp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 388201839, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 188, 194, 111, 125, 35, 23], OperandSize::Qword)
}

