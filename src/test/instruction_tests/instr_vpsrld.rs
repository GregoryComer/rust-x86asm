use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 114, 213, 98], OperandSize::Dword)
}

#[test]
fn vpsrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 114, 210, 41], OperandSize::Qword)
}

#[test]
fn vpsrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 114, 213, 105], OperandSize::Dword)
}

#[test]
fn vpsrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 114, 213, 1], OperandSize::Qword)
}

#[test]
fn vpsrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 101, 142, 114, 214, 110], OperandSize::Dword)
}

#[test]
fn vpsrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1327397176, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 140, 114, 148, 119, 56, 121, 30, 79, 57], OperandSize::Dword)
}

#[test]
fn vpsrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 2126213108, Some(OperandSize::Dword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 156, 114, 148, 151, 244, 111, 187, 126, 110], OperandSize::Dword)
}

#[test]
fn vpsrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM12)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 209, 93, 133, 114, 212, 121], OperandSize::Qword)
}

#[test]
fn vpsrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1496302023, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 5, 133, 114, 20, 213, 199, 193, 47, 89, 45], OperandSize::Qword)
}

#[test]
fn vpsrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM17)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 145, 114, 23, 12], OperandSize::Qword)
}

#[test]
fn vpsrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 114, 211, 101], OperandSize::Dword)
}

#[test]
fn vpsrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 697177026, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 117, 170, 114, 20, 205, 194, 19, 142, 41, 76], OperandSize::Dword)
}

#[test]
fn vpsrld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Four, 1286802126, Some(OperandSize::Dword), None)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 77, 190, 114, 148, 155, 206, 10, 179, 76, 106], OperandSize::Dword)
}

#[test]
fn vpsrld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 37, 162, 114, 212, 108], OperandSize::Qword)
}

#[test]
fn vpsrld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 45, 165, 114, 20, 80, 112], OperandSize::Qword)
}

#[test]
fn vpsrld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1511833539, Some(OperandSize::Dword), None)), operand3: Some(Literal8(4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 13, 190, 114, 148, 87, 195, 191, 28, 90, 4], OperandSize::Qword)
}

#[test]
fn vpsrld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 114, 212, 8], OperandSize::Dword)
}

#[test]
fn vpsrld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(ECX, 1834942271, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(123)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 206, 114, 145, 63, 255, 94, 109, 123], OperandSize::Dword)
}

#[test]
fn vpsrld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 69, 223, 114, 16, 0], OperandSize::Dword)
}

#[test]
fn vpsrld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM23)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 85, 207, 114, 215, 1], OperandSize::Qword)
}

#[test]
fn vpsrld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 194, 114, 20, 210, 66], OperandSize::Qword)
}

#[test]
fn vpsrld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1806035167, Some(OperandSize::Dword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 222, 114, 20, 141, 223, 232, 165, 107, 43], OperandSize::Qword)
}

#[test]
fn vpsrld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 210, 217], OperandSize::Dword)
}

#[test]
fn vpsrld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 210, 3], OperandSize::Dword)
}

#[test]
fn vpsrld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 210, 228], OperandSize::Qword)
}

#[test]
fn vpsrld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 2114802555, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 210, 140, 158, 123, 83, 13, 126], OperandSize::Qword)
}

#[test]
fn vpsrld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 210, 195], OperandSize::Dword)
}

#[test]
fn vpsrld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 210, 44, 87], OperandSize::Dword)
}

#[test]
fn vpsrld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 210, 234], OperandSize::Qword)
}

#[test]
fn vpsrld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 419459556, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 210, 52, 253, 228, 113, 0, 25], OperandSize::Qword)
}

#[test]
fn vpsrld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 140, 210, 238], OperandSize::Dword)
}

#[test]
fn vpsrld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 210, 36, 83], OperandSize::Dword)
}

#[test]
fn vpsrld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 29, 130, 210, 237], OperandSize::Qword)
}

#[test]
fn vpsrld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 129, 210, 52, 142], OperandSize::Qword)
}

#[test]
fn vpsrld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 210, 253], OperandSize::Dword)
}

#[test]
fn vpsrld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1198603127, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 210, 28, 189, 119, 59, 113, 71], OperandSize::Dword)
}

#[test]
fn vpsrld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 77, 167, 210, 192], OperandSize::Qword)
}

#[test]
fn vpsrld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RDX, 1700043800, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 101, 169, 210, 178, 24, 156, 84, 101], OperandSize::Qword)
}

#[test]
fn vpsrld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 206, 210, 214], OperandSize::Dword)
}

#[test]
fn vpsrld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 708619576, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 210, 20, 77, 56, 173, 60, 42], OperandSize::Dword)
}

#[test]
fn vpsrld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 109, 194, 210, 235], OperandSize::Qword)
}

#[test]
fn vpsrld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1219425874, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 125, 198, 210, 172, 222, 82, 246, 174, 72], OperandSize::Qword)
}

