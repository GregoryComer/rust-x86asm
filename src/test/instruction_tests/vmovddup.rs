use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 228], OperandSize::Dword)
}

fn vmovddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1972983849, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 132, 89, 41, 88, 153, 117], OperandSize::Dword)
}

fn vmovddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 208], OperandSize::Qword)
}

fn vmovddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RDI, 1204501571, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 159, 67, 60, 203, 71], OperandSize::Qword)
}

fn vmovddup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 208], OperandSize::Dword)
}

fn vmovddup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 551532030, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 172, 135, 254, 181, 223, 32], OperandSize::Dword)
}

fn vmovddup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 208], OperandSize::Qword)
}

fn vmovddup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 3], OperandSize::Qword)
}

fn vmovddup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 140, 18, 242], OperandSize::Dword)
}

fn vmovddup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 18, 4, 70], OperandSize::Dword)
}

fn vmovddup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 255, 143, 18, 248], OperandSize::Qword)
}

fn vmovddup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 218363103, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 255, 141, 18, 60, 253, 223, 244, 3, 13], OperandSize::Qword)
}

fn vmovddup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 169, 18, 228], OperandSize::Dword)
}

fn vmovddup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 18, 12, 217], OperandSize::Dword)
}

fn vmovddup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 255, 172, 18, 202], OperandSize::Qword)
}

fn vmovddup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM16)), operand2: Some(IndirectDisplaced(RAX, 1523219231, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 255, 173, 18, 128, 31, 123, 202, 90], OperandSize::Qword)
}

fn vmovddup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 18, 213], OperandSize::Dword)
}

fn vmovddup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 18, 20, 79], OperandSize::Dword)
}

fn vmovddup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 255, 201, 18, 209], OperandSize::Qword)
}

fn vmovddup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 255, 206, 18, 23], OperandSize::Qword)
}

