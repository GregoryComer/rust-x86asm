use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 95, 232], OperandSize::Dword)
}

#[test]
fn vmaxpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 401043166, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 95, 135, 222, 110, 231, 23], OperandSize::Dword)
}

#[test]
fn vmaxpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 95, 225], OperandSize::Qword)
}

#[test]
fn vmaxpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 811975206, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 95, 4, 157, 38, 194, 101, 48], OperandSize::Qword)
}

#[test]
fn vmaxpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 95, 223], OperandSize::Dword)
}

#[test]
fn vmaxpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 95, 44, 80], OperandSize::Dword)
}

#[test]
fn vmaxpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 95, 231], OperandSize::Qword)
}

#[test]
fn vmaxpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 1915004086, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 95, 132, 135, 182, 164, 36, 114], OperandSize::Qword)
}

#[test]
fn vmaxpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 205, 141, 95, 227], OperandSize::Dword)
}

#[test]
fn vmaxpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 142, 95, 26], OperandSize::Dword)
}

#[test]
fn vmaxpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Two, 1351448716, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 156, 95, 172, 113, 140, 120, 141, 80], OperandSize::Dword)
}

#[test]
fn vmaxpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 197, 131, 95, 209], OperandSize::Qword)
}

#[test]
fn vmaxpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 197, 142, 95, 52, 90], OperandSize::Qword)
}

#[test]
fn vmaxpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 203929494, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 181, 159, 95, 132, 222, 150, 183, 39, 12], OperandSize::Qword)
}

#[test]
fn vmaxpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 205, 172, 95, 226], OperandSize::Dword)
}

#[test]
fn vmaxpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1959564072, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 221, 173, 95, 175, 40, 147, 204, 116], OperandSize::Dword)
}

#[test]
fn vmaxpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 205, 186, 95, 18], OperandSize::Dword)
}

#[test]
fn vmaxpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 173, 165, 95, 225], OperandSize::Qword)
}

#[test]
fn vmaxpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDI, 1519454419, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 141, 167, 95, 191, 211, 8, 145, 90], OperandSize::Qword)
}

#[test]
fn vmaxpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 181, 189, 95, 63], OperandSize::Qword)
}

#[test]
fn vmaxpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 153, 95, 228], OperandSize::Dword)
}

#[test]
fn vmaxpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 168301796, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 95, 28, 213, 228, 20, 8, 10], OperandSize::Dword)
}

#[test]
fn vmaxpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1945758987, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 253, 223, 95, 20, 197, 11, 237, 249, 115], OperandSize::Dword)
}

#[test]
fn vmaxpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 205, 156, 95, 252], OperandSize::Qword)
}

#[test]
fn vmaxpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 189, 195, 95, 4, 211], OperandSize::Qword)
}

#[test]
fn vmaxpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectDisplaced(RDI, 1880429909, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 213, 214, 95, 183, 85, 21, 21, 112], OperandSize::Qword)
}

