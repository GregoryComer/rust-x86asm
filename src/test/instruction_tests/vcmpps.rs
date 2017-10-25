use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcmpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 194, 204, 59], OperandSize::Dword)
}

fn vcmpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 194, 48, 96], OperandSize::Dword)
}

fn vcmpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 194, 240, 51], OperandSize::Qword)
}

fn vcmpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 194, 14, 98], OperandSize::Qword)
}

fn vcmpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 194, 229, 109], OperandSize::Dword)
}

fn vcmpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1893093576, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 194, 4, 157, 200, 80, 214, 112, 52], OperandSize::Dword)
}

fn vcmpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 194, 213, 77], OperandSize::Qword)
}

fn vcmpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 78866865, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 194, 20, 149, 177, 105, 179, 4, 125], OperandSize::Qword)
}

fn vcmpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 14, 194, 213, 71], OperandSize::Dword)
}

fn vcmpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 1334429843, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 9, 194, 140, 81, 147, 200, 137, 79, 77], OperandSize::Dword)
}

fn vcmpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 28, 194, 62, 18], OperandSize::Dword)
}

fn vcmpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 12, 10, 194, 241, 38], OperandSize::Qword)
}

fn vcmpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 808729550, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 15, 194, 179, 206, 59, 52, 48, 77], OperandSize::Qword)
}

fn vcmpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 823152214, Some(OperandSize::Dword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 12, 30, 194, 60, 213, 86, 78, 16, 49, 123], OperandSize::Qword)
}

fn vcmpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(103)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 46, 194, 217, 103], OperandSize::Dword)
}

fn vcmpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 47, 194, 52, 215, 116], OperandSize::Dword)
}

fn vcmpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 58, 194, 59, 49], OperandSize::Dword)
}

fn vcmpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM22)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 177, 4, 45, 194, 214, 16], OperandSize::Qword)
}

fn vcmpps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 45, 194, 12, 192, 41], OperandSize::Qword)
}

fn vcmpps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RCX, 520011118, Some(OperandSize::Dword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 12, 61, 194, 145, 110, 189, 254, 30, 97], OperandSize::Qword)
}

fn vcmpps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 30, 194, 213, 97], OperandSize::Dword)
}

fn vcmpps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ECX, 1242040487, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 77, 194, 145, 167, 8, 8, 74, 54], OperandSize::Dword)
}

fn vcmpps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 1701524426, Some(OperandSize::Dword), None)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 90, 194, 166, 202, 51, 107, 101, 82], OperandSize::Dword)
}

fn vcmpps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(82)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 4, 19, 194, 234, 82], OperandSize::Qword)
}

fn vcmpps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 79, 194, 60, 150, 24], OperandSize::Qword)
}

fn vcmpps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPS, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 844775820, Some(OperandSize::Dword), None)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 116, 93, 194, 180, 219, 140, 65, 90, 50, 21], OperandSize::Qword)
}

