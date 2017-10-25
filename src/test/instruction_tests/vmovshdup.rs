use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovshdup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 197], OperandSize::Dword)
}

fn vmovshdup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 502039055, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 44, 125, 15, 130, 236, 29], OperandSize::Dword)
}

fn vmovshdup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 225], OperandSize::Qword)
}

fn vmovshdup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 814590866, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 22, 164, 192, 146, 171, 141, 48], OperandSize::Qword)
}

fn vmovshdup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 237], OperandSize::Dword)
}

fn vmovshdup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1351494310, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 4, 141, 166, 42, 142, 80], OperandSize::Dword)
}

fn vmovshdup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 204], OperandSize::Qword)
}

fn vmovshdup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 945910384, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 22, 140, 242, 112, 114, 97, 56], OperandSize::Qword)
}

fn vmovshdup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 138, 22, 213], OperandSize::Dword)
}

fn vmovshdup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 22, 24], OperandSize::Dword)
}

fn vmovshdup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 126, 137, 22, 222], OperandSize::Qword)
}

fn vmovshdup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(XMM21)), operand2: Some(IndirectDisplaced(RDX, 1015735196, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 126, 143, 22, 170, 156, 227, 138, 60], OperandSize::Qword)
}

fn vmovshdup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 169, 22, 248], OperandSize::Dword)
}

fn vmovshdup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 155253058, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 22, 28, 117, 66, 249, 64, 9], OperandSize::Dword)
}

fn vmovshdup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 126, 174, 22, 221], OperandSize::Qword)
}

fn vmovshdup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 65496789, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 22, 4, 205, 213, 102, 231, 3], OperandSize::Qword)
}

fn vmovshdup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 22, 238], OperandSize::Dword)
}

fn vmovshdup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 203, 22, 26], OperandSize::Dword)
}

fn vmovshdup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 126, 207, 22, 216], OperandSize::Qword)
}

fn vmovshdup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSHDUP, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 207, 22, 1], OperandSize::Qword)
}

