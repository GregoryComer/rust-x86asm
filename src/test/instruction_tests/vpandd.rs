use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 140, 219, 247], OperandSize::Dword)
}

fn vpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1101533382, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 143, 219, 36, 85, 198, 16, 168, 65], OperandSize::Dword)
}

fn vpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 156, 219, 55], OperandSize::Dword)
}

fn vpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 109, 140, 219, 218], OperandSize::Qword)
}

fn vpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 841711298, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 53, 137, 219, 36, 197, 194, 126, 43, 50], OperandSize::Qword)
}

fn vpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectDisplaced(RDX, 281531829, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 45, 145, 219, 138, 181, 213, 199, 16], OperandSize::Qword)
}

fn vpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 171, 219, 200], OperandSize::Dword)
}

fn vpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 84358205, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 109, 175, 219, 60, 181, 61, 52, 7, 5], OperandSize::Dword)
}

fn vpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2116557820, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 191, 219, 52, 221, 252, 27, 40, 126], OperandSize::Dword)
}

fn vpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 77, 163, 219, 215], OperandSize::Qword)
}

fn vpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM11)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 37, 171, 219, 23], OperandSize::Qword)
}

fn vpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM21)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 85, 183, 219, 2], OperandSize::Qword)
}

fn vpandd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 202, 219, 207], OperandSize::Dword)
}

fn vpandd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 807348473, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 204, 219, 4, 221, 249, 40, 31, 48], OperandSize::Dword)
}

fn vpandd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 220, 219, 17], OperandSize::Dword)
}

fn vpandd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 61, 203, 219, 218], OperandSize::Qword)
}

fn vpandd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RCX, 386157571, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 109, 196, 219, 169, 3, 76, 4, 23], OperandSize::Qword)
}

fn vpandd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 45, 218, 219, 27], OperandSize::Qword)
}

