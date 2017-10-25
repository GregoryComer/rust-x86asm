use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn ficom_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 17], OperandSize::Word)
}

fn ficom_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 17], OperandSize::Dword)
}

fn ficom_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectDisplaced(RCX, 1748107556, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[218, 145, 36, 1, 50, 104], OperandSize::Qword)
}

fn ficom_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectDisplaced(SI, 192, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 148, 192, 0], OperandSize::Word)
}

fn ficom_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectScaledDisplaced(ESI, Two, 1197855841, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 20, 117, 97, 212, 101, 71], OperandSize::Dword)
}

fn ficom_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FICOM, operand1: Some(IndirectDisplaced(RBX, 747862296, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[222, 147, 24, 121, 147, 44], OperandSize::Qword)
}

