use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcmppd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 194, 212, 64], OperandSize::Dword)
}

#[test]
fn vcmppd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(29)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 194, 3, 29], OperandSize::Dword)
}

#[test]
fn vcmppd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(53)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 194, 210, 53], OperandSize::Qword)
}

#[test]
fn vcmppd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 464140853, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(64)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 194, 156, 206, 53, 58, 170, 27, 64], OperandSize::Qword)
}

#[test]
fn vcmppd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 194, 208, 70], OperandSize::Dword)
}

#[test]
fn vcmppd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1107629475, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 194, 132, 193, 163, 21, 5, 66, 28], OperandSize::Dword)
}

#[test]
fn vcmppd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 194, 214, 113], OperandSize::Qword)
}

#[test]
fn vcmppd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 255776349, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 194, 28, 157, 93, 214, 62, 15, 98], OperandSize::Qword)
}

#[test]
fn vcmppd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 205, 9, 194, 231, 58], OperandSize::Dword)
}

#[test]
fn vcmppd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Two, 33374637, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(38)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 12, 194, 140, 87, 173, 65, 253, 1, 38], OperandSize::Dword)
}

#[test]
fn vcmppd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 736941839, Some(OperandSize::Qword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 245, 27, 194, 139, 15, 215, 236, 43, 52], OperandSize::Dword)
}

#[test]
fn vcmppd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM18)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 133, 1, 194, 210, 108], OperandSize::Qword)
}

#[test]
fn vcmppd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Four, 455942089, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 221, 7, 194, 180, 130, 201, 31, 45, 27, 104], OperandSize::Qword)
}

#[test]
fn vcmppd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1980378979, Some(OperandSize::Qword), None)), operand4: Some(Literal8(108)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 237, 23, 194, 12, 253, 99, 47, 10, 118, 108], OperandSize::Qword)
}

#[test]
fn vcmppd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 197, 43, 194, 210, 5], OperandSize::Dword)
}

#[test]
fn vcmppd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 44, 194, 27, 24], OperandSize::Dword)
}

#[test]
fn vcmppd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1048589321, Some(OperandSize::Qword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 62, 194, 186, 9, 52, 128, 62, 124], OperandSize::Dword)
}

#[test]
fn vcmppd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM15)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 173, 35, 194, 255, 71], OperandSize::Qword)
}

#[test]
fn vcmppd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 205, 35, 194, 52, 198, 51], OperandSize::Qword)
}

#[test]
fn vcmppd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 253, 58, 194, 44, 94, 57], OperandSize::Qword)
}

#[test]
fn vcmppd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 229, 31, 194, 227, 23], OperandSize::Dword)
}

#[test]
fn vcmppd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 188722242, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(49)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 73, 194, 148, 91, 66, 172, 63, 11, 49], OperandSize::Dword)
}

#[test]
fn vcmppd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 474508086, Some(OperandSize::Qword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 213, 93, 194, 176, 54, 107, 72, 28, 77], OperandSize::Dword)
}

#[test]
fn vcmppd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM12)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 253, 19, 194, 236, 124], OperandSize::Qword)
}

#[test]
fn vcmppd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1860263564, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 74, 194, 52, 69, 140, 94, 225, 110, 115], OperandSize::Qword)
}

#[test]
fn vcmppd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VCMPPD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Four, 124053130, Some(OperandSize::Qword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 181, 91, 194, 180, 135, 138, 230, 100, 7, 60], OperandSize::Qword)
}

