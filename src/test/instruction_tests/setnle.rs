use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn setnle_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Word)
}

fn setnle_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 383, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 130, 127, 1], OperandSize::Word)
}

fn setnle_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 194], OperandSize::Dword)
}

fn setnle_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectDisplaced(ECX, 930575370, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 129, 10, 116, 119, 55], OperandSize::Dword)
}

fn setnle_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 193], OperandSize::Qword)
}

fn setnle_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNLE, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 159, 4, 199], OperandSize::Qword)
}

