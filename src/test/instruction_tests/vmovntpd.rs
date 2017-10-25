use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 810499222, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 20, 205, 150, 60, 79, 48], OperandSize::Dword)
}

fn vmovntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 60, 199], OperandSize::Qword)
}

fn vmovntpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 22], OperandSize::Dword)
}

fn vmovntpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RDI, Two, 1259584799, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 60, 125, 31, 189, 19, 75], OperandSize::Qword)
}

fn vmovntpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 1753304973, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 20, 77, 141, 79, 129, 104], OperandSize::Dword)
}

fn vmovntpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1507176841, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 52, 133, 137, 177, 213, 89], OperandSize::Qword)
}

fn vmovntpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(EBX, 2127802275, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 131, 163, 175, 211, 126], OperandSize::Dword)
}

fn vmovntpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RCX, Four, 84653210, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 40, 43, 52, 141, 154, 180, 11, 5], OperandSize::Qword)
}

fn vmovntpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1857211797, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 36, 189, 149, 205, 178, 110], OperandSize::Dword)
}

fn vmovntpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 577948270, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 72, 43, 164, 201, 110, 202, 114, 34], OperandSize::Qword)
}

