use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 102, 198], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ESI, 223689888, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 102, 182, 160, 60, 85, 13], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 102, 193], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1957070178, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 102, 180, 248, 98, 133, 166, 116], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 102, 248], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 102, 36, 122], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 102, 226], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 102, 55], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 13, 102, 247], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 13, 102, 34], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EDX, 1603033158, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 27, 102, 146, 70, 88, 140, 95], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 109, 15, 102, 208], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM12)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 15, 102, 23], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 21, 17, 102, 12, 178], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 46, 102, 249], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 46, 102, 20, 113], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 1920658446, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 61, 102, 188, 208, 14, 236, 122, 114], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 39, 102, 243], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1557362988, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 36, 102, 20, 69, 44, 121, 211, 92], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 855936059, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 21, 57, 102, 12, 221, 59, 140, 4, 51], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 79, 102, 219], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1977761865, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 117, 73, 102, 44, 221, 73, 64, 226, 117], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1622354844, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 92, 102, 60, 197, 156, 43, 179, 96], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 78, 102, 230], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 21, 65, 102, 35], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1326939119, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 61, 84, 102, 44, 125, 239, 123, 23, 79], OperandSize::Qword)
}

