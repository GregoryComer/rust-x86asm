use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psignd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 202], OperandSize::Dword)
}

fn psignd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1849278432, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 52, 77, 224, 191, 57, 110], OperandSize::Dword)
}

fn psignd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 229], OperandSize::Qword)
}

fn psignd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1124458248, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 10, 140, 203, 8, 223, 5, 67], OperandSize::Qword)
}

fn psignd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 208], OperandSize::Dword)
}

fn psignd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EDI, 418659061, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 167, 245, 58, 244, 24], OperandSize::Dword)
}

fn psignd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 192], OperandSize::Qword)
}

fn psignd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSIGND, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 10, 60, 216], OperandSize::Qword)
}

