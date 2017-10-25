use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vshufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 198, 214, 17], OperandSize::Dword)
}

fn vshufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 349173930, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 198, 164, 80, 170, 248, 207, 20, 41], OperandSize::Dword)
}

fn vshufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 235, 40], OperandSize::Qword)
}

fn vshufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 38, 1], OperandSize::Qword)
}

fn vshufps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 198, 195, 118], OperandSize::Dword)
}

fn vshufps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 957061663, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 198, 132, 222, 31, 154, 11, 57, 85], OperandSize::Dword)
}

fn vshufps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 198, 192, 79], OperandSize::Qword)
}

fn vshufps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1561597113, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 198, 52, 245, 185, 20, 20, 93, 119], OperandSize::Qword)
}

fn vshufps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 143, 198, 238, 44], OperandSize::Dword)
}

fn vshufps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 39743356, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 140, 198, 182, 124, 111, 94, 2, 71], OperandSize::Dword)
}

fn vshufps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 336682474, Some(OperandSize::Dword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 154, 198, 36, 125, 234, 93, 17, 20, 112], OperandSize::Dword)
}

fn vshufps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 193, 20, 134, 198, 241, 29], OperandSize::Qword)
}

fn vshufps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RBX, 766199169, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 92, 132, 198, 163, 129, 69, 171, 45, 29], OperandSize::Qword)
}

fn vshufps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(55)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 12, 147, 198, 7, 55], OperandSize::Qword)
}

fn vshufps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 170, 198, 223, 87], OperandSize::Dword)
}

fn vshufps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 175, 198, 34, 118], OperandSize::Dword)
}

fn vshufps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 70439757, Some(OperandSize::Dword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 190, 198, 60, 213, 77, 211, 50, 4, 112], OperandSize::Dword)
}

fn vshufps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM27)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 60, 161, 198, 251, 95], OperandSize::Qword)
}

fn vshufps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 76, 175, 198, 20, 211, 63], OperandSize::Qword)
}

fn vshufps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectDisplaced(RCX, 1753683696, Some(OperandSize::Dword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 92, 179, 198, 129, 240, 22, 135, 104, 82], OperandSize::Qword)
}

fn vshufps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(95)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 76, 206, 198, 213, 95], OperandSize::Dword)
}

fn vshufps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ECX, 751587303, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 206, 198, 137, 231, 79, 204, 44, 121], OperandSize::Dword)
}

fn vshufps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 625868567, Some(OperandSize::Dword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 223, 198, 140, 215, 23, 255, 77, 37, 49], OperandSize::Dword)
}

fn vshufps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM15)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 68, 202, 198, 247, 10], OperandSize::Qword)
}

fn vshufps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 279404639, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 203, 198, 12, 197, 95, 96, 167, 16, 89], OperandSize::Qword)
}

fn vshufps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RDI, 1658769990, Some(OperandSize::Dword), None)), operand4: Some(Literal8(80)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 4, 212, 198, 135, 70, 210, 222, 98, 80], OperandSize::Qword)
}

