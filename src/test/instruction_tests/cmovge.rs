use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovge_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 250], OperandSize::Word)
}

fn cmovge_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 242, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 179, 242, 0], OperandSize::Word)
}

fn cmovge_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 253], OperandSize::Dword)
}

fn cmovge_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DI)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 56], OperandSize::Dword)
}

fn cmovge_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 214], OperandSize::Qword)
}

fn cmovge_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(SI)), operand2: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 50], OperandSize::Qword)
}

fn cmovge_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 254], OperandSize::Word)
}

fn cmovge_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 6607, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 77, 160, 207, 25], OperandSize::Word)
}

fn cmovge_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 213], OperandSize::Dword)
}

fn cmovge_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1252883053, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 148, 240, 109, 122, 173, 74], OperandSize::Dword)
}

fn cmovge_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 242], OperandSize::Qword)
}

fn cmovge_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 77, 60, 66], OperandSize::Qword)
}

fn cmovge_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 233], OperandSize::Qword)
}

fn cmovge_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVGE, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 454511084, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 77, 148, 152, 236, 73, 23, 27], OperandSize::Qword)
}

