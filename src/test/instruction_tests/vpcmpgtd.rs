use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 102, 211], OperandSize::Dword)
}

fn vpcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EDX, 1218604448, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 102, 162, 160, 109, 162, 72], OperandSize::Dword)
}

fn vpcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 102, 211], OperandSize::Qword)
}

fn vpcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1482022222, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 102, 155, 78, 221, 85, 88], OperandSize::Qword)
}

fn vpcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 102, 225], OperandSize::Dword)
}

fn vpcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 102, 18], OperandSize::Dword)
}

fn vpcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 102, 218], OperandSize::Qword)
}

fn vpcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 102, 43], OperandSize::Qword)
}

fn vpcmpgtd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 14, 102, 242], OperandSize::Dword)
}

fn vpcmpgtd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1663969910, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 13, 102, 188, 112, 118, 42, 46, 99], OperandSize::Dword)
}

fn vpcmpgtd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1397134384, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 29, 102, 52, 93, 48, 148, 70, 83], OperandSize::Dword)
}

fn vpcmpgtd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 29, 11, 102, 217], OperandSize::Qword)
}

fn vpcmpgtd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 593499848, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 5, 14, 102, 164, 144, 200, 22, 96, 35], OperandSize::Qword)
}

fn vpcmpgtd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 25, 102, 12, 177], OperandSize::Qword)
}

fn vpcmpgtd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 41, 102, 216], OperandSize::Dword)
}

fn vpcmpgtd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 848823733, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 41, 102, 12, 149, 181, 5, 152, 50], OperandSize::Dword)
}

fn vpcmpgtd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 109, 62, 102, 49], OperandSize::Dword)
}

fn vpcmpgtd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 46, 102, 212], OperandSize::Qword)
}

fn vpcmpgtd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 1435582631, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 45, 34, 102, 140, 71, 167, 64, 145, 85], OperandSize::Qword)
}

fn vpcmpgtd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1020131884, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 53, 61, 102, 156, 159, 44, 250, 205, 60], OperandSize::Qword)
}

fn vpcmpgtd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 76, 102, 215], OperandSize::Dword)
}

fn vpcmpgtd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 651183665, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 77, 102, 140, 243, 49, 70, 208, 38], OperandSize::Dword)
}

fn vpcmpgtd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 91, 102, 60, 139], OperandSize::Dword)
}

fn vpcmpgtd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 101, 75, 102, 227], OperandSize::Qword)
}

fn vpcmpgtd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 53, 66, 102, 31], OperandSize::Qword)
}

fn vpcmpgtd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 498119469, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 37, 82, 102, 180, 209, 45, 179, 176, 29], OperandSize::Qword)
}

