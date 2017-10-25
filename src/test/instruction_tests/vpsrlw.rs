use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsrlw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 113, 209, 50], OperandSize::Dword)
}

fn vpsrlw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(117)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 113, 211, 117], OperandSize::Qword)
}

fn vpsrlw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 214, 47], OperandSize::Dword)
}

fn vpsrlw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 113, 208, 36], OperandSize::Qword)
}

fn vpsrlw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 143, 113, 214, 28], OperandSize::Dword)
}

fn vpsrlw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 799202649, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 138, 113, 20, 77, 89, 221, 162, 47, 21], OperandSize::Dword)
}

fn vpsrlw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM10)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 85, 135, 113, 210, 22], OperandSize::Qword)
}

fn vpsrlw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 1354803427, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 140, 113, 148, 210, 227, 168, 192, 80, 87], OperandSize::Qword)
}

fn vpsrlw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 113, 210, 58], OperandSize::Dword)
}

fn vpsrlw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 801901155, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 85, 174, 113, 148, 115, 99, 10, 204, 47, 98], OperandSize::Dword)
}

fn vpsrlw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM19)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 37, 163, 113, 211, 3], OperandSize::Qword)
}

fn vpsrlw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 743321742, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 162, 113, 148, 127, 142, 48, 78, 44, 62], OperandSize::Qword)
}

fn vpsrlw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 125, 201, 113, 211, 76], OperandSize::Dword)
}

fn vpsrlw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 44711139, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 93, 205, 113, 148, 83, 227, 60, 170, 2, 110], OperandSize::Dword)
}

fn vpsrlw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM7)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 207, 113, 215, 3], OperandSize::Qword)
}

fn vpsrlw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 93, 203, 113, 17, 3], OperandSize::Qword)
}

fn vpsrlw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 209, 196], OperandSize::Dword)
}

fn vpsrlw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 209, 28, 208], OperandSize::Dword)
}

fn vpsrlw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 209, 196], OperandSize::Qword)
}

fn vpsrlw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1857882310, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 209, 52, 245, 198, 8, 189, 110], OperandSize::Qword)
}

fn vpsrlw_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 209, 254], OperandSize::Dword)
}

fn vpsrlw_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 592732632, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 209, 172, 242, 216, 97, 84, 35], OperandSize::Dword)
}

fn vpsrlw_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 209, 206], OperandSize::Qword)
}

fn vpsrlw_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 15782300, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 209, 140, 151, 156, 209, 240, 0], OperandSize::Qword)
}

fn vpsrlw_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 209, 206], OperandSize::Dword)
}

fn vpsrlw_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 1227934620, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 139, 209, 185, 156, 203, 48, 73], OperandSize::Dword)
}

fn vpsrlw_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 69, 141, 209, 226], OperandSize::Qword)
}

fn vpsrlw_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 101, 137, 209, 35], OperandSize::Qword)
}

fn vpsrlw_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 171, 209, 193], OperandSize::Dword)
}

fn vpsrlw_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1160510534, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 209, 4, 189, 70, 252, 43, 69], OperandSize::Dword)
}

fn vpsrlw_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 37, 171, 209, 227], OperandSize::Qword)
}

fn vpsrlw_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 1371154534, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 61, 161, 209, 188, 72, 102, 40, 186, 81], OperandSize::Qword)
}

fn vpsrlw_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 209, 205], OperandSize::Dword)
}

fn vpsrlw_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 203, 209, 52, 67], OperandSize::Dword)
}

fn vpsrlw_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 93, 197, 209, 193], OperandSize::Qword)
}

fn vpsrlw_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RAX, 176669828, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 13, 197, 209, 152, 132, 196, 135, 10], OperandSize::Qword)
}

