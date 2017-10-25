use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vfnmadd231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 188, 210], OperandSize::Dword)
}

fn vfnmadd231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1383787518, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 188, 180, 119, 254, 235, 122, 82], OperandSize::Dword)
}

fn vfnmadd231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 188, 217], OperandSize::Qword)
}

fn vfnmadd231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RDX, 322570669, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 188, 162, 173, 9, 58, 19], OperandSize::Qword)
}

fn vfnmadd231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 188, 252], OperandSize::Dword)
}

fn vfnmadd231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 188, 19], OperandSize::Dword)
}

fn vfnmadd231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 188, 246], OperandSize::Qword)
}

fn vfnmadd231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 448932026, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 188, 190, 186, 40, 194, 26], OperandSize::Qword)
}

fn vfnmadd231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 142, 188, 226], OperandSize::Dword)
}

fn vfnmadd231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 188, 12, 202], OperandSize::Dword)
}

fn vfnmadd231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 294776781, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 156, 188, 52, 221, 205, 239, 145, 17], OperandSize::Dword)
}

fn vfnmadd231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 69, 130, 188, 200], OperandSize::Qword)
}

fn vfnmadd231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 61958213, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 29, 133, 188, 28, 181, 69, 104, 177, 3], OperandSize::Qword)
}

fn vfnmadd231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 296431945, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 93, 159, 188, 156, 147, 73, 49, 171, 17], OperandSize::Qword)
}

fn vfnmadd231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 174, 188, 217], OperandSize::Dword)
}

fn vfnmadd231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 734127218, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 175, 188, 52, 157, 114, 228, 193, 43], OperandSize::Dword)
}

fn vfnmadd231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 188, 188, 62], OperandSize::Dword)
}

fn vfnmadd231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 188, 229], OperandSize::Qword)
}

fn vfnmadd231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 101, 171, 188, 20, 182], OperandSize::Qword)
}

fn vfnmadd231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 13, 189, 188, 28, 211], OperandSize::Qword)
}

fn vfnmadd231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 93, 252, 188, 243], OperandSize::Dword)
}

fn vfnmadd231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 205, 188, 8], OperandSize::Dword)
}

fn vfnmadd231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 2016740371, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 219, 188, 172, 80, 19, 4, 53, 120], OperandSize::Dword)
}

fn vfnmadd231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 101, 213, 188, 196], OperandSize::Qword)
}

fn vfnmadd231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RDI, 373360844, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 77, 199, 188, 159, 204, 8, 65, 22], OperandSize::Qword)
}

fn vfnmadd231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1977291230, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 45, 210, 188, 180, 240, 222, 17, 219, 117], OperandSize::Qword)
}

