use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 114, 212, 44], OperandSize::Dword)
}

#[test]
fn vpsrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 209, 9], OperandSize::Qword)
}

#[test]
fn vpsrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 114, 208, 99], OperandSize::Dword)
}

#[test]
fn vpsrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 114, 208, 50], OperandSize::Qword)
}

#[test]
fn vpsrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 114, 211, 83], OperandSize::Dword)
}

#[test]
fn vpsrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1471130983, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 139, 114, 148, 80, 103, 173, 175, 87, 21], OperandSize::Dword)
}

#[test]
fn vpsrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 477842738, Some(OperandSize::Dword), None)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 125, 159, 114, 20, 93, 50, 77, 123, 28, 55], OperandSize::Dword)
}

#[test]
fn vpsrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM25)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 109, 137, 114, 209, 99], OperandSize::Qword)
}

#[test]
fn vpsrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 29, 130, 114, 20, 206, 80], OperandSize::Qword)
}

#[test]
fn vpsrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 2136580631, Some(OperandSize::Dword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 53, 149, 114, 20, 69, 23, 162, 89, 127, 105], OperandSize::Qword)
}

#[test]
fn vpsrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 171, 114, 211, 118], OperandSize::Dword)
}

#[test]
fn vpsrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 114, 20, 248, 107], OperandSize::Dword)
}

#[test]
fn vpsrld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(ECX, 2041041227, Some(OperandSize::Dword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 188, 114, 145, 75, 209, 167, 121, 4], OperandSize::Dword)
}

#[test]
fn vpsrld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM24)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 61, 169, 114, 208, 6], OperandSize::Qword)
}

#[test]
fn vpsrld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 114, 20, 183, 61], OperandSize::Qword)
}

#[test]
fn vpsrld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 838673114, Some(OperandSize::Dword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 29, 180, 114, 20, 125, 218, 34, 253, 49, 78], OperandSize::Qword)
}

#[test]
fn vpsrld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 205, 114, 209, 91], OperandSize::Dword)
}

#[test]
fn vpsrld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 202, 114, 20, 250, 50], OperandSize::Dword)
}

#[test]
fn vpsrld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 2015547017, Some(OperandSize::Dword), None)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 222, 114, 20, 253, 137, 206, 34, 120, 75], OperandSize::Dword)
}

#[test]
fn vpsrld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM8)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 61, 203, 114, 208, 30], OperandSize::Qword)
}

#[test]
fn vpsrld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM27)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1172569737, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 37, 196, 114, 20, 125, 137, 254, 227, 69, 81], OperandSize::Qword)
}

#[test]
fn vpsrld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectDisplaced(RBX, 281078259, Some(OperandSize::Dword), None)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 5, 223, 114, 147, 243, 233, 192, 16, 121], OperandSize::Qword)
}

#[test]
fn vpsrld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 210, 198], OperandSize::Dword)
}

#[test]
fn vpsrld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 336830640, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 210, 140, 154, 176, 160, 19, 20], OperandSize::Dword)
}

#[test]
fn vpsrld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 210, 238], OperandSize::Qword)
}

#[test]
fn vpsrld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1382643180, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 210, 28, 69, 236, 117, 105, 82], OperandSize::Qword)
}

#[test]
fn vpsrld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 210, 233], OperandSize::Dword)
}

#[test]
fn vpsrld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 210, 44, 118], OperandSize::Dword)
}

#[test]
fn vpsrld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 210, 220], OperandSize::Qword)
}

#[test]
fn vpsrld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 210, 1], OperandSize::Qword)
}

#[test]
fn vpsrld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 210, 225], OperandSize::Dword)
}

#[test]
fn vpsrld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(ESI, 1136423069, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 138, 210, 158, 157, 112, 188, 67], OperandSize::Dword)
}

#[test]
fn vpsrld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 93, 142, 210, 247], OperandSize::Qword)
}

#[test]
fn vpsrld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 21, 129, 210, 4, 186], OperandSize::Qword)
}

#[test]
fn vpsrld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 210, 222], OperandSize::Dword)
}

#[test]
fn vpsrld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDI, 839536798, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 171, 210, 175, 158, 80, 10, 50], OperandSize::Dword)
}

#[test]
fn vpsrld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 53, 167, 210, 234], OperandSize::Qword)
}

#[test]
fn vpsrld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 45, 167, 210, 52, 178], OperandSize::Qword)
}

#[test]
fn vpsrld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 202, 210, 231], OperandSize::Dword)
}

#[test]
fn vpsrld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 947246654, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 206, 210, 60, 213, 62, 214, 117, 56], OperandSize::Dword)
}

#[test]
fn vpsrld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 61, 205, 210, 202], OperandSize::Qword)
}

#[test]
fn vpsrld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 514279647, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 29, 201, 210, 44, 149, 223, 72, 167, 30], OperandSize::Qword)
}

