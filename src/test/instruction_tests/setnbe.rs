use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Word)
}

fn setnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 1], OperandSize::Word)
}

fn setnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Dword)
}

fn setnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 73], OperandSize::Dword)
}

fn setnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

fn setnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 263027877, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 132, 223, 165, 124, 173, 15], OperandSize::Qword)
}

fn setnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

fn setnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 203], OperandSize::Qword)
}

