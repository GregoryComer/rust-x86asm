use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 190, 195], OperandSize::Dword)
}

fn vfnmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 190, 44, 158], OperandSize::Dword)
}

fn vfnmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 190, 227], OperandSize::Qword)
}

fn vfnmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 190, 36, 146], OperandSize::Qword)
}

fn vfnmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 190, 236], OperandSize::Dword)
}

fn vfnmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 719924605, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 190, 44, 77, 125, 45, 233, 42], OperandSize::Dword)
}

fn vfnmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 190, 237], OperandSize::Qword)
}

fn vfnmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RSI, 1520076983, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 190, 142, 183, 136, 154, 90], OperandSize::Qword)
}

fn vfnmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 142, 190, 220], OperandSize::Dword)
}

fn vfnmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 2070734307, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 190, 20, 157, 227, 229, 108, 123], OperandSize::Dword)
}

fn vfnmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 153, 190, 28, 118], OperandSize::Dword)
}

fn vfnmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 5, 133, 190, 203], OperandSize::Qword)
}

fn vfnmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1959736321, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 69, 130, 190, 140, 217, 1, 52, 207, 116], OperandSize::Qword)
}

fn vfnmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 109, 156, 190, 19], OperandSize::Qword)
}

fn vfnmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 190, 212], OperandSize::Dword)
}

fn vfnmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1738268618, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 190, 20, 125, 202, 223, 155, 103], OperandSize::Dword)
}

fn vfnmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 1021247876, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 185, 190, 180, 194, 132, 1, 223, 60], OperandSize::Dword)
}

fn vfnmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 101, 164, 190, 233], OperandSize::Qword)
}

fn vfnmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 166, 190, 36, 249], OperandSize::Qword)
}

fn vfnmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 1647467737, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 29, 188, 190, 36, 77, 217, 92, 50, 98], OperandSize::Qword)
}

fn vfnmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 188, 190, 243], OperandSize::Dword)
}

fn vfnmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EBX, 2053290893, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 206, 190, 131, 141, 187, 98, 122], OperandSize::Dword)
}

fn vfnmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 190, 43], OperandSize::Dword)
}

fn vfnmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 53, 146, 190, 248], OperandSize::Qword)
}

fn vfnmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 45, 199, 190, 52, 153], OperandSize::Qword)
}

fn vfnmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 209, 190, 52, 65], OperandSize::Qword)
}

