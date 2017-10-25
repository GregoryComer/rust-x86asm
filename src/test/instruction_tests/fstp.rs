use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fstp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 27], OperandSize::Word)
}

fn fstp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 31], OperandSize::Dword)
}

fn fstp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Two, 1626701133, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 156, 120, 77, 125, 245, 96], OperandSize::Qword)
}

fn fstp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 4243, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 186, 147, 16], OperandSize::Word)
}

fn fstp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 60, 248], OperandSize::Dword)
}

fn fstp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 826106398, Some(OperandSize::Tbyte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 188, 248, 30, 98, 61, 49], OperandSize::Qword)
}

fn fstp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 31], OperandSize::Word)
}

fn fstp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 30], OperandSize::Dword)
}

fn fstp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 28, 202], OperandSize::Qword)
}

fn fstp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST2)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 218], OperandSize::Word)
}

fn fstp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST3)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 219], OperandSize::Dword)
}

fn fstp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTP, operand1: Some(Direct(ST5)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[221, 221], OperandSize::Qword)
}

