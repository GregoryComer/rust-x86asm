use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 207], OperandSize::Word)
}

fn cmovp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(SI, 1, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 116, 1], OperandSize::Word)
}

fn cmovp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 233], OperandSize::Dword)
}

fn cmovp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1567451528, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 60, 125, 136, 105, 109, 93], OperandSize::Dword)
}

fn cmovp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 225], OperandSize::Qword)
}

fn cmovp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 20, 250], OperandSize::Qword)
}

fn cmovp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 250], OperandSize::Word)
}

fn cmovp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 15954, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 74, 163, 82, 62], OperandSize::Word)
}

fn cmovp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 247], OperandSize::Dword)
}

fn cmovp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 728619748, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 180, 215, 228, 218, 109, 43], OperandSize::Dword)
}

fn cmovp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 209], OperandSize::Qword)
}

fn cmovp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(EBX)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 74, 25], OperandSize::Qword)
}

fn cmovp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 215], OperandSize::Qword)
}

fn cmovp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVP, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1092277039, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 74, 172, 144, 47, 211, 26, 65], OperandSize::Qword)
}

