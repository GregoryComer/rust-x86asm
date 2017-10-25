use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 0, 211], OperandSize::Dword)
}

fn vpshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 711219087, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 0, 167, 143, 87, 100, 42], OperandSize::Dword)
}

fn vpshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 0, 227], OperandSize::Qword)
}

fn vpshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 0, 32], OperandSize::Qword)
}

fn vpshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 0, 222], OperandSize::Dword)
}

fn vpshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 0, 4, 218], OperandSize::Dword)
}

fn vpshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 0, 194], OperandSize::Qword)
}

fn vpshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RCX, 1913529069, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 0, 169, 237, 34, 14, 114], OperandSize::Qword)
}

fn vpshufb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 141, 0, 202], OperandSize::Dword)
}

fn vpshufb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 61880039, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 0, 132, 202, 231, 54, 176, 3], OperandSize::Dword)
}

fn vpshufb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 109, 129, 0, 211], OperandSize::Qword)
}

fn vpshufb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1320708385, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 93, 137, 0, 140, 94, 33, 105, 184, 78], OperandSize::Qword)
}

fn vpshufb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 0, 235], OperandSize::Dword)
}

fn vpshufb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 967561479, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 0, 156, 145, 7, 209, 171, 57], OperandSize::Dword)
}

fn vpshufb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 53, 161, 0, 211], OperandSize::Qword)
}

fn vpshufb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RBX, 122918456, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 163, 0, 163, 56, 150, 83, 7], OperandSize::Qword)
}

fn vpshufb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 0, 199], OperandSize::Dword)
}

fn vpshufb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 622759622, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 0, 176, 198, 142, 30, 37], OperandSize::Dword)
}

fn vpshufb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 61, 195, 0, 219], OperandSize::Qword)
}

fn vpshufb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 45, 202, 0, 33], OperandSize::Qword)
}

