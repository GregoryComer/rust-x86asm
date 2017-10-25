use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fdivr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(DI, 28140, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 189, 236, 109], OperandSize::Word)
}

fn fdivr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 57], OperandSize::Dword)
}

fn fdivr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectDisplaced(RSI, 1423051957, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 190, 181, 12, 210, 84], OperandSize::Qword)
}

fn fdivr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 254], OperandSize::Word)
}

fn fdivr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 250], OperandSize::Dword)
}

fn fdivr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST)), operand2: Some(Direct(ST4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[216, 252], OperandSize::Qword)
}

fn fdivr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 21364, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 187, 116, 83], OperandSize::Word)
}

fn fdivr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1899818989, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 60, 125, 237, 239, 60, 113], OperandSize::Dword)
}

fn fdivr_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1505640817, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 188, 82, 113, 65, 190, 89], OperandSize::Qword)
}

fn fdivr_10() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 244], OperandSize::Word)
}

fn fdivr_11() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 244], OperandSize::Dword)
}

fn fdivr_12() {
    run_test(&Instruction { mnemonic: Mnemonic::FDIVR, operand1: Some(Direct(ST4)), operand2: Some(Direct(ST)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[220, 244], OperandSize::Qword)
}

