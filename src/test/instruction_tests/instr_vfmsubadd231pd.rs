use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 183, 245], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 183, 42], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 183, 227], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 183, 44, 240], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 183, 197], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1718621703, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 183, 20, 221, 7, 22, 112, 102], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 183, 245], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 183, 4, 79], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 142, 183, 204], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 183, 33], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 183, 58], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 197, 135, 183, 210], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 141, 129, 183, 42], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RSI, 1315061135, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 149, 155, 183, 190, 143, 61, 98, 78], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 173, 183, 255], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 183, 35], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 286212464, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 187, 183, 188, 72, 112, 65, 15, 17], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 213, 165, 183, 200], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RSI, 1483661384, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 245, 165, 183, 174, 72, 224, 110, 88], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 189, 191, 183, 33], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 189, 183, 222], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 523367304, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 204, 183, 60, 117, 136, 243, 49, 31], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 183, 34], OperandSize::Dword)
}

#[test]
fn vfmsubadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 253, 245, 183, 229], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectDisplaced(RDI, 979477358, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 229, 195, 183, 191, 110, 163, 97, 58], OperandSize::Qword)
}

#[test]
fn vfmsubadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD231PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 189, 219, 183, 48], OperandSize::Qword)
}

