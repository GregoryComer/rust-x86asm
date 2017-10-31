use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 114, 209, 29], OperandSize::Dword)
}

#[test]
fn vpsrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 114, 215, 2], OperandSize::Qword)
}

#[test]
fn vpsrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 114, 211, 9], OperandSize::Dword)
}

#[test]
fn vpsrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 114, 209, 25], OperandSize::Qword)
}

#[test]
fn vpsrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 114, 210, 91], OperandSize::Dword)
}

#[test]
fn vpsrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Two, 1313279497, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 142, 114, 148, 89, 9, 14, 71, 78, 35], OperandSize::Dword)
}

#[test]
fn vpsrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 101, 153, 114, 18, 116], OperandSize::Dword)
}

#[test]
fn vpsrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM10)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 5, 130, 114, 210, 127], OperandSize::Qword)
}

#[test]
fn vpsrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1882065627, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(60)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 13, 142, 114, 20, 189, 219, 10, 46, 112, 60], OperandSize::Qword)
}

#[test]
fn vpsrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Two, 560057391, Some(OperandSize::Dword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 153, 114, 148, 71, 47, 204, 97, 33, 90], OperandSize::Qword)
}

#[test]
fn vpsrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 114, 212, 3], OperandSize::Dword)
}

#[test]
fn vpsrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 69, 170, 114, 20, 183, 100], OperandSize::Dword)
}

#[test]
fn vpsrld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 129539144, Some(OperandSize::Dword), None)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 85, 189, 114, 148, 95, 72, 156, 184, 7, 89], OperandSize::Dword)
}

#[test]
fn vpsrld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM18)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 53, 171, 114, 210, 18], OperandSize::Qword)
}

#[test]
fn vpsrld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 101, 165, 114, 20, 71, 5], OperandSize::Qword)
}

#[test]
fn vpsrld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 187, 114, 20, 186, 35], OperandSize::Qword)
}

#[test]
fn vpsrld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 109, 201, 114, 214, 57], OperandSize::Dword)
}

#[test]
fn vpsrld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 207, 114, 20, 113, 93], OperandSize::Dword)
}

#[test]
fn vpsrld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 218, 114, 20, 255, 108], OperandSize::Dword)
}

#[test]
fn vpsrld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM2)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 199, 114, 210, 96], OperandSize::Qword)
}

#[test]
fn vpsrld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(71)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 5, 201, 114, 20, 203, 71], OperandSize::Qword)
}

#[test]
fn vpsrld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 629882894, Some(OperandSize::Dword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 53, 213, 114, 20, 221, 14, 64, 139, 37, 49], OperandSize::Qword)
}

#[test]
fn vpsrld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 210, 222], OperandSize::Dword)
}

#[test]
fn vpsrld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1323281370, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 210, 60, 125, 218, 171, 223, 78], OperandSize::Dword)
}

#[test]
fn vpsrld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 210, 201], OperandSize::Qword)
}

#[test]
fn vpsrld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 1277909765, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 210, 128, 5, 91, 43, 76], OperandSize::Qword)
}

#[test]
fn vpsrld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 210, 228], OperandSize::Dword)
}

#[test]
fn vpsrld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 210, 1], OperandSize::Dword)
}

#[test]
fn vpsrld_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 210, 201], OperandSize::Qword)
}

#[test]
fn vpsrld_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 210, 27], OperandSize::Qword)
}

#[test]
fn vpsrld_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 139, 210, 252], OperandSize::Dword)
}

#[test]
fn vpsrld_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 108652458, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 210, 12, 213, 170, 231, 121, 6], OperandSize::Dword)
}

#[test]
fn vpsrld_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 145, 69, 129, 210, 192], OperandSize::Qword)
}

#[test]
fn vpsrld_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 53, 134, 210, 44, 81], OperandSize::Qword)
}

#[test]
fn vpsrld_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 171, 210, 235], OperandSize::Dword)
}

#[test]
fn vpsrld_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 85, 172, 210, 12, 151], OperandSize::Dword)
}

#[test]
fn vpsrld_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 117, 167, 210, 196], OperandSize::Qword)
}

#[test]
fn vpsrld_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 93, 169, 210, 20, 145], OperandSize::Qword)
}

#[test]
fn vpsrld_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 204, 210, 247], OperandSize::Dword)
}

#[test]
fn vpsrld_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 202, 210, 12, 254], OperandSize::Dword)
}

#[test]
fn vpsrld_41() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 69, 193, 210, 203], OperandSize::Qword)
}

#[test]
fn vpsrld_42() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 37, 206, 210, 12, 223], OperandSize::Qword)
}

