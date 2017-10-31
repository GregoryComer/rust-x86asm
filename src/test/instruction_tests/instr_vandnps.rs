use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandnps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 85, 210], OperandSize::Dword)
}

#[test]
fn vandnps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 85, 4, 242], OperandSize::Dword)
}

#[test]
fn vandnps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 85, 236], OperandSize::Qword)
}

#[test]
fn vandnps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RBX, 1316766987, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 85, 163, 11, 69, 124, 78], OperandSize::Qword)
}

#[test]
fn vandnps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 85, 236], OperandSize::Dword)
}

#[test]
fn vandnps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1090406715, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 85, 148, 119, 59, 73, 254, 64], OperandSize::Dword)
}

#[test]
fn vandnps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 85, 222], OperandSize::Qword)
}

#[test]
fn vandnps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 85, 4, 122], OperandSize::Qword)
}

#[test]
fn vandnps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 138, 85, 245], OperandSize::Dword)
}

#[test]
fn vandnps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 116, 139, 85, 0], OperandSize::Dword)
}

#[test]
fn vandnps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 158, 85, 60, 201], OperandSize::Dword)
}

#[test]
fn vandnps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 84, 134, 85, 217], OperandSize::Qword)
}

#[test]
fn vandnps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 116, 143, 85, 4, 82], OperandSize::Qword)
}

#[test]
fn vandnps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDX, 1709637431, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 92, 153, 85, 154, 55, 255, 230, 101], OperandSize::Qword)
}

#[test]
fn vandnps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 116, 175, 85, 202], OperandSize::Dword)
}

#[test]
fn vandnps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 173, 85, 20, 247], OperandSize::Dword)
}

#[test]
fn vandnps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EBX, 1406631154, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 186, 85, 171, 242, 124, 215, 83], OperandSize::Dword)
}

#[test]
fn vandnps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 116, 175, 85, 242], OperandSize::Qword)
}

#[test]
fn vandnps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 1612127952, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 100, 164, 85, 132, 146, 208, 30, 23, 96], OperandSize::Qword)
}

#[test]
fn vandnps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectDisplaced(RDI, 1420351028, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 12, 190, 85, 143, 52, 214, 168, 84], OperandSize::Qword)
}

#[test]
fn vandnps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 206, 85, 245], OperandSize::Dword)
}

#[test]
fn vandnps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1820730545, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 202, 85, 156, 254, 177, 36, 134, 108], OperandSize::Dword)
}

#[test]
fn vandnps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 924034986, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 219, 85, 60, 253, 170, 167, 19, 55], OperandSize::Dword)
}

#[test]
fn vandnps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 17, 124, 194, 85, 212], OperandSize::Qword)
}

#[test]
fn vandnps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 20, 201, 85, 12, 198], OperandSize::Qword)
}

#[test]
fn vandnps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDNPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1185494447, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 60, 214, 85, 52, 117, 175, 53, 169, 70], OperandSize::Qword)
}

