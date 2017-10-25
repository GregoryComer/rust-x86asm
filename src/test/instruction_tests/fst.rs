use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fst_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(SI, 27410, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 148, 18, 107], OperandSize::Word)
}

fn fst_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 18], OperandSize::Dword)
}

fn fst_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 20, 120], OperandSize::Qword)
}

fn fst_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 25363, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 146, 19, 99], OperandSize::Word)
}

fn fst_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 22], OperandSize::Dword)
}

fn fst_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(IndirectDisplaced(RCX, 1118081957, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 145, 165, 147, 164, 66], OperandSize::Qword)
}

fn fst_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 214], OperandSize::Word)
}

fn fst_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 214], OperandSize::Dword)
}

fn fst_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FST, operand1: Some(Direct(ST4)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 212], OperandSize::Qword)
}

