use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 170, 229], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 170, 62], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 170, 238], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 170, 38], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 170, 241], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 170, 54], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 170, 227], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 50570983, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 170, 164, 246, 231, 166, 3, 3], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 143, 170, 193], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 143, 170, 49], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 1145530499, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 159, 170, 148, 154, 131, 104, 71, 68], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 101, 142, 170, 206], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1911944461, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 29, 129, 170, 60, 197, 13, 245, 245, 113], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectDisplaced(RSI, 1618891292, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 45, 158, 170, 190, 28, 82, 126, 96], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 170, 198], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 172, 170, 41], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 191, 170, 20, 190], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 167, 170, 235], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 360982520, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 93, 174, 170, 36, 221, 248, 39, 132, 21], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1693284705, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 69, 178, 170, 164, 209, 97, 121, 237, 100], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 218, 170, 208], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EDI, 1904092565, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 170, 183, 149, 37, 126, 113], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 170, 20, 95], OperandSize::Dword)
}

#[test]
fn vfmsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 250, 170, 212], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 109, 206, 170, 36, 126], OperandSize::Qword)
}

#[test]
fn vfmsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB213PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 425337153, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 5, 217, 170, 148, 88, 65, 33, 90, 25], OperandSize::Qword)
}

