use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(DI, 18069, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 133, 149, 70], OperandSize::Word)
}

fn fld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledDisplaced(EAX, Two, 269631217, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 4, 69, 241, 62, 18, 16], OperandSize::Dword)
}

fn fld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 4, 240], OperandSize::Qword)
}

fn fld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 193], OperandSize::Word)
}

fn fld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST1)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 193], OperandSize::Dword)
}

fn fld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(Direct(ST6)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 198], OperandSize::Qword)
}

fn fld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 43], OperandSize::Word)
}

fn fld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 44, 202], OperandSize::Dword)
}

fn fld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectDisplaced(RAX, 2006233434, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 168, 90, 177, 148, 119], OperandSize::Qword)
}

fn fld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 148, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 131, 148, 0], OperandSize::Word)
}

fn fld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 4, 254], OperandSize::Dword)
}

fn fld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FLD, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1562734643, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 4, 125, 51, 112, 37, 93], OperandSize::Qword)
}

