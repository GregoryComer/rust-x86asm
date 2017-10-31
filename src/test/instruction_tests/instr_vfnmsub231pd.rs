use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 190, 219], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 190, 8], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 190, 241], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 190, 52, 251], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 190, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 190, 44, 184], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 190, 245], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 190, 15], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 190, 220], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 317295503, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 190, 52, 189, 143, 139, 233, 18], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 153, 190, 4, 137], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 181, 134, 190, 222], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM30)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 141, 134, 190, 25], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM29)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 149, 145, 190, 25], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 197, 174, 190, 243], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1239850612, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 190, 52, 85, 116, 158, 230, 73], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 188, 190, 28, 82], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 133, 162, 190, 218], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 181, 162, 190, 58], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM15)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 133, 187, 190, 16], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 186, 190, 249], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1238484675, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 205, 190, 28, 93, 195, 198, 209, 73], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1111698260, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 223, 190, 188, 249, 84, 43, 67, 66], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 229, 245, 190, 215], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM15)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 133, 201, 190, 55], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1886716264, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 229, 211, 190, 44, 189, 104, 1, 117, 112], OperandSize::Qword)
}

