use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 13, 31, 231, 30], OperandSize::Dword)
}

fn vpcmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDX, 236277198, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 69, 13, 31, 138, 206, 77, 21, 14, 48], OperandSize::Dword)
}

fn vpcmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(91)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 28, 31, 12, 83, 91], OperandSize::Dword)
}

fn vpcmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 93, 5, 31, 246, 34], OperandSize::Qword)
}

fn vpcmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 820651835, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 5, 13, 31, 140, 223, 59, 39, 234, 48, 71], OperandSize::Qword)
}

fn vpcmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 93, 20, 31, 25, 54], OperandSize::Qword)
}

fn vpcmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 109, 46, 31, 207, 85], OperandSize::Dword)
}

fn vpcmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 338815932, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 69, 43, 31, 148, 176, 188, 235, 49, 20, 30], OperandSize::Dword)
}

fn vpcmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 85, 62, 31, 12, 206, 24], OperandSize::Dword)
}

fn vpcmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 211, 13, 47, 31, 238, 8], OperandSize::Qword)
}

fn vpcmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RBX, 1085401962, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 53, 41, 31, 163, 106, 235, 177, 64, 14], OperandSize::Qword)
}

fn vpcmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 29, 60, 31, 36, 216, 115], OperandSize::Qword)
}

fn vpcmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 79, 31, 242, 88], OperandSize::Dword)
}

fn vpcmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 117, 73, 31, 52, 158, 116], OperandSize::Dword)
}

fn vpcmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(99)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 89, 31, 28, 251, 99], OperandSize::Dword)
}

fn vpcmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM31)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 147, 109, 68, 31, 247, 5], OperandSize::Qword)
}

fn vpcmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1921104017, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(107)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 69, 76, 31, 28, 117, 145, 184, 129, 114, 107], OperandSize::Qword)
}

fn vpcmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 45, 90, 31, 20, 73, 42], OperandSize::Qword)
}

