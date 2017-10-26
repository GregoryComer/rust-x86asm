use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 154, 255], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1811475260, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 154, 28, 221, 60, 235, 248, 107], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 154, 246], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 2145465546, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 154, 4, 125, 202, 52, 225, 127], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 154, 254], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 2054957252, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 154, 4, 85, 196, 40, 124, 122], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 154, 202], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 690332465, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 154, 28, 157, 49, 163, 37, 41], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 140, 154, 240], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 328776563, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 154, 188, 187, 115, 187, 152, 19], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 157, 154, 62], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 213, 143, 154, 250], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 92906094, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 205, 143, 154, 4, 245, 110, 162, 137, 5], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1329395334, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 157, 159, 154, 148, 88, 134, 246, 60, 79], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 154, 248], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 213665526, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 169, 154, 156, 254, 246, 70, 188, 12], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 250110943, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 191, 154, 158, 223, 99, 232, 14], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 149, 173, 154, 227], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 369841616, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 229, 171, 154, 52, 197, 208, 85, 11, 22], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Two, 1355888743, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 221, 187, 154, 132, 118, 103, 56, 209, 80], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 186, 154, 222], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Two, 1270745174, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 204, 154, 164, 126, 86, 8, 190, 75], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ESI, 1168351114, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 217, 154, 166, 138, 159, 163, 69], OperandSize::Dword)
}

#[test]
fn vfmsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 197, 155, 154, 239], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 205584020, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 197, 194, 154, 60, 157, 148, 246, 64, 12], OperandSize::Qword)
}

#[test]
fn vfmsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(RAX, 1378599629, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 221, 154, 176, 205, 194, 43, 82], OperandSize::Qword)
}

