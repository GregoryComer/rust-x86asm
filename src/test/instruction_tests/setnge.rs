use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Word)
}

fn setnge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 31443, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 128, 211, 122], OperandSize::Word)
}

fn setnge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 194], OperandSize::Dword)
}

fn setnge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectDisplaced(EAX, 118562625, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 128, 65, 31, 17, 7], OperandSize::Dword)
}

fn setnge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 193], OperandSize::Qword)
}

fn setnge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 4, 218], OperandSize::Qword)
}

fn setnge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 195], OperandSize::Qword)
}

fn setnge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNGE, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 156, 0], OperandSize::Qword)
}

