use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn div_1() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 241], OperandSize::Word)
}

fn div_2() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(BP, 91, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 118, 91], OperandSize::Word)
}

fn div_3() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Dword)
}

fn div_4() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 541220569, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 180, 241, 217, 94, 66, 32], OperandSize::Dword)
}

fn div_5() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 241], OperandSize::Qword)
}

fn div_6() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledDisplaced(RBX, Two, 862934917, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 52, 93, 133, 87, 111, 51], OperandSize::Qword)
}

fn div_7() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 242], OperandSize::Qword)
}

fn div_8() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RDI, 2080591597, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 183, 237, 78, 3, 124], OperandSize::Qword)
}

fn div_9() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(SI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 246], OperandSize::Word)
}

fn div_10() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Memory(31564, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 54, 76, 123], OperandSize::Word)
}

fn div_11() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 242], OperandSize::Dword)
}

fn div_12() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(EDI, 1516506722, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 183, 98, 14, 100, 90], OperandSize::Dword)
}

fn div_13() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(BX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 243], OperandSize::Qword)
}

fn div_14() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RAX, 828081587, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 176, 179, 133, 91, 49], OperandSize::Qword)
}

fn div_15() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(ESI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 246], OperandSize::Word)
}

fn div_16() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(SI, 11498, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 180, 234, 44], OperandSize::Word)
}

fn div_17() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 247], OperandSize::Dword)
}

fn div_18() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 51], OperandSize::Dword)
}

fn div_19() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 245], OperandSize::Qword)
}

fn div_20() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1061232990, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 180, 151, 94, 33, 65, 63], OperandSize::Qword)
}

fn div_21() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(Direct(RDX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 242], OperandSize::Qword)
}

fn div_22() {
    run_test(&Instruction { mnemonic: Mnemonic::DIV, operand1: Some(IndirectDisplaced(RAX, 1331516658, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 176, 242, 84, 93, 79], OperandSize::Qword)
}

