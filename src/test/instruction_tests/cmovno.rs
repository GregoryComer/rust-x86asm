use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn cmovno_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 238], OperandSize::Word)
}

fn cmovno_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DX)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 23], OperandSize::Word)
}

fn cmovno_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 230], OperandSize::Dword)
}

fn cmovno_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(ECX, 354104798, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 177, 222, 53, 27, 21], OperandSize::Dword)
}

fn cmovno_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 249], OperandSize::Qword)
}

fn cmovno_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1396882650, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 28, 157, 218, 188, 66, 83], OperandSize::Qword)
}

fn cmovno_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 250], OperandSize::Word)
}

fn cmovno_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 65, 49], OperandSize::Word)
}

fn cmovno_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 244], OperandSize::Dword)
}

fn cmovno_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(ESI, 1121325925, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 182, 101, 19, 214, 66], OperandSize::Dword)
}

fn cmovno_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 202], OperandSize::Qword)
}

fn cmovno_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1931771833, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 65, 180, 192, 185, 127, 36, 115], OperandSize::Qword)
}

fn cmovno_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 213], OperandSize::Qword)
}

fn cmovno_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNO, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 65, 36, 66], OperandSize::Qword)
}

