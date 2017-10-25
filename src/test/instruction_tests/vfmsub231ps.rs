use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 186, 215], OperandSize::Dword)
}

fn vfmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 188028227, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 186, 132, 83, 67, 21, 53, 11], OperandSize::Dword)
}

fn vfmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 186, 241], OperandSize::Qword)
}

fn vfmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 676547680, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 186, 20, 125, 96, 76, 83, 40], OperandSize::Qword)
}

fn vfmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 186, 227], OperandSize::Dword)
}

fn vfmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 2120212932, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 186, 188, 150, 196, 225, 95, 126], OperandSize::Dword)
}

fn vfmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 186, 198], OperandSize::Qword)
}

fn vfmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1745495347, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 186, 172, 206, 51, 37, 10, 104], OperandSize::Qword)
}

fn vfmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 186, 234], OperandSize::Dword)
}

fn vfmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1136079127, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 186, 188, 118, 23, 49, 183, 67], OperandSize::Dword)
}

fn vfmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 154, 186, 3], OperandSize::Dword)
}

fn vfmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 85, 131, 186, 239], OperandSize::Qword)
}

fn vfmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 2017742964, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 21, 133, 186, 4, 85, 116, 80, 68, 120], OperandSize::Qword)
}

fn vfmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1220432031, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 154, 186, 180, 191, 159, 80, 190, 72], OperandSize::Qword)
}

fn vfmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 186, 198], OperandSize::Dword)
}

fn vfmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 186, 16], OperandSize::Dword)
}

fn vfmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 10017772, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 186, 36, 157, 236, 219, 152, 0], OperandSize::Dword)
}

fn vfmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 5, 170, 186, 196], OperandSize::Qword)
}

fn vfmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 166, 186, 20, 71], OperandSize::Qword)
}

fn vfmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RBX, 1662546993, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 13, 179, 186, 147, 49, 116, 24, 99], OperandSize::Qword)
}

fn vfmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 249, 186, 214], OperandSize::Dword)
}

fn vfmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1981569009, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 186, 172, 144, 241, 87, 28, 118], OperandSize::Dword)
}

fn vfmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 243827095, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 217, 186, 20, 253, 151, 129, 136, 14], OperandSize::Dword)
}

fn vfmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 61, 249, 186, 213], OperandSize::Qword)
}

fn vfmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectDisplaced(RSI, 1685467035, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 69, 199, 186, 166, 155, 47, 118, 100], OperandSize::Qword)
}

fn vfmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1456703658, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 223, 186, 164, 129, 170, 136, 211, 86], OperandSize::Qword)
}

