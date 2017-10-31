use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 188, 226], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1586527808, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 188, 12, 157, 64, 126, 144, 94], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 188, 239], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 188, 35], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 188, 232], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1750253028, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 188, 164, 250, 228, 189, 82, 104], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 188, 203], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 188, 59], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 188, 245], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 188, 4, 217], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 50283311, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 159, 188, 28, 77, 47, 67, 255, 2], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 197, 139, 188, 219], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectDisplaced(RSI, 1068596394, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 197, 135, 188, 142, 170, 124, 177, 63], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 150, 188, 20, 254], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 188, 221], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1906776513, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 188, 188, 250, 193, 25, 167, 113], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 673693232, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 188, 28, 205, 48, 190, 39, 40], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 221, 162, 188, 195], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 181, 172, 188, 60, 240], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RCX, 77446251, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 173, 181, 188, 137, 107, 188, 157, 4], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 249, 188, 225], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 698334587, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 188, 36, 117, 123, 189, 159, 41], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 222, 188, 32], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 197, 222, 188, 235], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 759804081, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 237, 197, 188, 52, 85, 177, 176, 73, 45], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 157, 209, 188, 2], OperandSize::Qword)
}

