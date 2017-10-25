use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmaddsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 166, 233], OperandSize::Dword)
}

fn vfmaddsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 166, 63], OperandSize::Dword)
}

fn vfmaddsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 166, 217], OperandSize::Qword)
}

fn vfmaddsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 166, 28, 67], OperandSize::Qword)
}

fn vfmaddsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 166, 209], OperandSize::Dword)
}

fn vfmaddsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 983076956, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 166, 164, 158, 92, 144, 152, 58], OperandSize::Dword)
}

fn vfmaddsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 166, 198], OperandSize::Qword)
}

fn vfmaddsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RSI, 1917459116, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 166, 158, 172, 26, 74, 114], OperandSize::Qword)
}

fn vfmaddsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 143, 166, 227], OperandSize::Dword)
}

fn vfmaddsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 166, 14], OperandSize::Dword)
}

fn vfmaddsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 159, 166, 33], OperandSize::Dword)
}

fn vfmaddsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 165, 141, 166, 248], OperandSize::Qword)
}

fn vfmaddsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1804227253, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 165, 137, 166, 180, 75, 181, 82, 138, 107], OperandSize::Qword)
}

fn vfmaddsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectDisplaced(RCX, 1687812416, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 133, 147, 166, 177, 64, 249, 153, 100], OperandSize::Qword)
}

fn vfmaddsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 166, 210], OperandSize::Dword)
}

fn vfmaddsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 166, 11], OperandSize::Dword)
}

fn vfmaddsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDI, 1882690242, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 189, 166, 159, 194, 146, 55, 112], OperandSize::Dword)
}

fn vfmaddsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 213, 169, 166, 224], OperandSize::Qword)
}

fn vfmaddsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 173, 169, 166, 20, 186], OperandSize::Qword)
}

fn vfmaddsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 977417918, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 187, 166, 60, 85, 190, 54, 66, 58], OperandSize::Qword)
}

fn vfmaddsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 158, 166, 199], OperandSize::Dword)
}

fn vfmaddsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1250127919, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 203, 166, 36, 117, 47, 112, 131, 74], OperandSize::Dword)
}

fn vfmaddsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1010367407, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 166, 12, 221, 175, 251, 56, 60], OperandSize::Dword)
}

fn vfmaddsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 253, 215, 166, 210], OperandSize::Qword)
}

fn vfmaddsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1651199567, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 237, 198, 166, 36, 93, 79, 78, 107, 98], OperandSize::Qword)
}

fn vfmaddsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 430632691, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 237, 223, 166, 188, 80, 243, 238, 170, 25], OperandSize::Qword)
}

