use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpeqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 118, 193], OperandSize::Dword)
}

fn vpcmpeqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 100411419, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 118, 147, 27, 40, 252, 5], OperandSize::Dword)
}

fn vpcmpeqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 118, 236], OperandSize::Qword)
}

fn vpcmpeqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1601805692, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 118, 60, 253, 124, 157, 121, 95], OperandSize::Qword)
}

fn vpcmpeqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 118, 250], OperandSize::Dword)
}

fn vpcmpeqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EBX, 1617757333, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 118, 131, 149, 4, 109, 96], OperandSize::Dword)
}

fn vpcmpeqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 118, 219], OperandSize::Qword)
}

fn vpcmpeqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RBX, 291692561, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 118, 187, 17, 224, 98, 17], OperandSize::Qword)
}

fn vpcmpeqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 14, 118, 200], OperandSize::Dword)
}

fn vpcmpeqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 11, 118, 60, 202], OperandSize::Dword)
}

fn vpcmpeqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 2120951703, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 31, 118, 145, 151, 39, 107, 126], OperandSize::Dword)
}

fn vpcmpeqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 85, 10, 118, 237], OperandSize::Qword)
}

fn vpcmpeqd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 371767797, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 11, 118, 12, 197, 245, 185, 40, 22], OperandSize::Qword)
}

fn vpcmpeqd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectDisplaced(RSI, 83788190, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 109, 20, 118, 182, 158, 129, 254, 4], OperandSize::Qword)
}

fn vpcmpeqd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 42, 118, 204], OperandSize::Dword)
}

fn vpcmpeqd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1788294615, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 43, 118, 188, 126, 215, 53, 151, 106], OperandSize::Dword)
}

fn vpcmpeqd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 425007614, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 61, 118, 52, 141, 254, 25, 85, 25], OperandSize::Dword)
}

fn vpcmpeqd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 85, 33, 118, 205], OperandSize::Qword)
}

fn vpcmpeqd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 422388926, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 53, 43, 118, 164, 118, 190, 36, 45, 25], OperandSize::Qword)
}

fn vpcmpeqd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 63, 118, 60, 187], OperandSize::Qword)
}

fn vpcmpeqd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 93, 79, 118, 202], OperandSize::Dword)
}

fn vpcmpeqd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 992064064, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 79, 118, 60, 85, 64, 178, 33, 59], OperandSize::Dword)
}

fn vpcmpeqd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ECX, 784043010, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 90, 118, 177, 2, 140, 187, 46], OperandSize::Dword)
}

fn vpcmpeqd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 37, 68, 118, 210], OperandSize::Qword)
}

fn vpcmpeqd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 5, 68, 118, 18], OperandSize::Qword)
}

fn vpcmpeqd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 238878167, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 93, 84, 118, 36, 221, 215, 253, 60, 14], OperandSize::Qword)
}

