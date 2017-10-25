use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpavgb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 224, 230], OperandSize::Dword)
}

fn vpavgb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 446734090, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 224, 4, 213, 10, 159, 160, 26], OperandSize::Dword)
}

fn vpavgb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 224, 252], OperandSize::Qword)
}

fn vpavgb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 1028040399, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 224, 171, 207, 166, 70, 61], OperandSize::Qword)
}

fn vpavgb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 224, 239], OperandSize::Dword)
}

fn vpavgb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 224, 44, 217], OperandSize::Dword)
}

fn vpavgb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 224, 220], OperandSize::Qword)
}

fn vpavgb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1606434343, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 224, 140, 126, 39, 62, 192, 95], OperandSize::Qword)
}

fn vpavgb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 142, 224, 226], OperandSize::Dword)
}

fn vpavgb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 438954003, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 224, 164, 95, 19, 232, 41, 26], OperandSize::Dword)
}

fn vpavgb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 85, 132, 224, 199], OperandSize::Qword)
}

fn vpavgb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectDisplaced(RBX, 1381939445, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 37, 139, 224, 187, 245, 184, 94, 82], OperandSize::Qword)
}

fn vpavgb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 171, 224, 208], OperandSize::Dword)
}

fn vpavgb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDX, 1429247138, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 170, 224, 146, 162, 148, 48, 85], OperandSize::Dword)
}

fn vpavgb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 85, 167, 224, 243], OperandSize::Qword)
}

fn vpavgb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 192488193, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 162, 224, 148, 209, 1, 35, 121, 11], OperandSize::Qword)
}

fn vpavgb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 207, 224, 200], OperandSize::Dword)
}

fn vpavgb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 224, 40], OperandSize::Dword)
}

fn vpavgb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 45, 195, 224, 220], OperandSize::Qword)
}

fn vpavgb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPAVGB, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM13)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 21, 205, 224, 47], OperandSize::Qword)
}

